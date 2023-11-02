#include "passenger_list_model.h"
#include "car/car_controller.h"
#include "event_dispatcher.h"
#include "passenger/passenger_controller.h"
#include <QCoroTask>

using namespace Simple::Controller;

PassengerListModel::PassengerListModel(QObject *parent) : QAbstractListModel(parent)
{

    connect(EventDispatcher::instance()->car(), &CarSignals::allRelationsInvalidated, this, [this](int carId) {
        if (carId != m_carId)
        {
            return;
        }
        auto task = Simple::Controller::Car::CarController::instance()->getWithDetails(carId);
        QCoro::connect(std::move(task), this, [this, carId](auto &&carDetails) {
            if (!carDetails.isValid())
            {
                return;
            }
            QList<PassengerDTO> newPassengers = carDetails.passengers();

            // first, add the missing passengers

            // we have new passengers
            for (const auto &passenger : newPassengers)
            {
                if (!m_passengerList.contains(passenger))
                {
                    // add the passenger
                    int row = m_passengerList.size();
                    beginInsertRows(QModelIndex(), row, row);
                    m_passengerList.append(passenger);
                    m_passengerIdList.append(passenger.id());
                    endInsertRows();
                }
            }

            // then, remove the passengers that are not in the new list

            for (int i = m_passengerList.size() - 1; i >= 0; --i)
            {
                if (!newPassengers.contains(m_passengerList[i]))
                {
                    // remove the passenger
                    beginRemoveRows(QModelIndex(), i, i);
                    m_passengerList.removeAt(i);
                    m_passengerIdList.removeAt(i);
                    endRemoveRows();
                }
            }
            // then, move the current passagers so the list is in the same order as the new list

            for (int i = 0; i < m_passengerList.size(); ++i)
            {
                if (m_passengerList[i] != newPassengers[i])
                {
                    // move the passenger
                    int row = newPassengers.indexOf(m_passengerList[i]);
                    beginMoveRows(QModelIndex(), i, i, QModelIndex(), row);
                    m_passengerList.move(i, row);
                    m_passengerIdList.move(i, row);
                    endMoveRows();
                }
            }

            // finally, update the passengers that are in both lists if the updateDateDate has changed

            for (int i = 0; i < m_passengerList.size(); ++i)
            {
                if (m_passengerList[i].updateDate() != newPassengers[i].updateDate())
                {
                    // update the passenger
                    m_passengerList[i] = newPassengers[i];
                    QModelIndex topLeft = index(i, 0);
                    QModelIndex bottomRight = index(i, 0);
                    emit dataChanged(topLeft, bottomRight);
                }
            }

            return;
        });
    });

    // TODO: replace with relationRemoved
    connect(EventDispatcher::instance()->car(), &CarSignals::relationRemoved, this, [this](CarRelationDTO dto) {
        if (dto.relationField() != CarRelationDTO::RelationField::Passengers)
        {
            return;
        }

        // remove the passenger list
        QList<int> relatedIds = dto.relatedIds();

        for (int id : relatedIds)
        {
            if (!m_passengerIdList.contains(id))
            {
                continue;
            }

            int index = m_passengerIdList.indexOf(id);
            if (index != -1)
            {
                beginRemoveRows(QModelIndex(), index, index);
                m_passengerList.removeAt(index);
                m_passengerIdList.removeAt(index);
                endRemoveRows();
            }
        }
    });

    connect(EventDispatcher::instance()->car(), &CarSignals::relationInserted, this, [this](CarRelationDTO dto) {
        if (dto.id() != m_carId || dto.relationField() != CarRelationDTO::RelationField::Passengers)
        {
            return;
        }

        int position = dto.position();

        // reverse dto.relatedIds()
        QList<int> relatedIds = dto.relatedIds();
        std::reverse(relatedIds.begin(), relatedIds.end());

        // fetch passengers from controller
        for (int passengerId : relatedIds)
        {
            Passenger::PassengerController::instance()
                ->get(passengerId)
                .then([this, passengerId, position](PassengerDTO passenger) {
                    // add passengers to this model
                    if (!m_passengerIdList.contains(passengerId))
                    {
                        beginInsertRows(QModelIndex(), position, position);
                        m_passengerList.insert(position, passenger);
                        m_passengerIdList.insert(position, passengerId);
                        endInsertRows();
                    }
                });
        }
    });

    connect(EventDispatcher::instance()->passenger(), &PassengerSignals::updated, this, [this](PassengerDTO dto) {
        for (int i = 0; i < m_passengerList.size(); ++i)
        {
            if (m_passengerList.at(i).id() == dto.id())
            {
                m_passengerList[i] = dto;
                m_passengerIdList[i] = dto.id();
                emit dataChanged(index(i), index(i));
                break;
            }
        }
    });
}

QVariant PassengerListModel::headerData(int section, Qt::Orientation orientation, int role) const
{
    return QVariant();
}

int PassengerListModel::rowCount(const QModelIndex &parent) const
{
    // For list models only the root node (an invalid parent) should return the list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not become a tree model.
    if (parent.isValid())
        return 0;

    return m_passengerList.count();
}

QVariant PassengerListModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

    int row = index.row();
    if (row >= m_passengerList.size())
        return QVariant();

    if (role == Qt::DisplayRole)
    {
        return m_passengerList.at(row).name();
    }

    if (role == Id)
        return m_passengerList.at(row).id();
    if (role == Name)
        return m_passengerList.at(row).name();

    return QVariant();
}

Qt::ItemFlags PassengerListModel::flags(const QModelIndex &index) const
{
    if (!index.isValid())
        return Qt::NoItemFlags;

    return Qt::ItemIsEditable | QAbstractItemModel::flags(index);
}

bool PassengerListModel::setData(const QModelIndex &index, const QVariant &value, int role)
{
    if (!index.isValid())
        return false;

    int row = index.row();
    if (row >= m_passengerList.size())
        return false;

    if (role == Qt::EditRole)
    {
        m_passengerList[row].setName(value.toString());

        UpdatePassengerDTO dto;
        dto.setId(m_passengerList[row].id());
        dto.setName(m_passengerList[row].name());

        Passenger::PassengerController::instance()->update(dto);

        emit dataChanged(index, index, {role});
        return true;
    }

    return false;
}

void PassengerListModel::populate()
{
    if (m_carId == 0)
        return;

    auto task = Car::CarController::instance()->getWithDetails(m_carId);
    QCoro::connect(std::move(task), this, [this](auto &&result) {
        const QList<Simple::Contracts::DTO::Passenger::PassengerDTO> &passengers = result.passengers();
        beginInsertRows(QModelIndex(), 0, passengers.size() - 1);
        m_passengerList.clear();
        m_passengerIdList.clear();
        m_passengerList = passengers;
        // fill m_passengerIdList
        for (const auto &passenger : passengers)
        {
            m_passengerIdList.append(passenger.id());
        }

        endInsertRows();
    });
}

int PassengerListModel::carId() const
{
    return m_carId;
}

void PassengerListModel::setCarId(int newCarId)
{
    if (m_carId == newCarId)
        return;
    m_carId = newCarId;

    if (m_carId == 0)
    {
        beginResetModel();
        m_passengerList.clear();
        m_passengerIdList.clear();
        endResetModel();
    }
    else
    {
        populate();
    }
    emit carIdChanged();
}

void PassengerListModel::resetCarId()
{
    setCarId(0);
}

QHash<int, QByteArray> PassengerListModel::roleNames() const
{
    QHash<int, QByteArray> names;
    names[Id] = "itemId";
    names[Name] = "name";
    return names;
}
