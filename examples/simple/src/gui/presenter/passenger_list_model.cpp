#include "passenger_list_model.h"
#include "car/car_controller.h"
#include "event_dispatcher.h"
#include "passenger/passenger_controller.h"
#include <QCoroTask>

using namespace Simple::Controller;

PassengerListModel::PassengerListModel(QObject *parent) : QAbstractListModel(parent)
{

    connect(EventDispatcher::instance()->car(), &CarSignals::detailsUpdated, this, [this](int carId) {
        if (carId == m_carId)
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
                if (!m_passengers.contains(passenger))
                {
                    // add the passenger
                    int row = m_passengers.size();
                    beginInsertRows(QModelIndex(), row, row);
                    m_passengers.append(passenger);
                    endInsertRows();
                }
            }

            // then, remove the passengers that are not in the new list

            for (int i = m_passengers.size() - 1; i >= 0; --i)
            {
                if (!newPassengers.contains(m_passengers[i]))
                {
                    // remove the passenger
                    beginRemoveRows(QModelIndex(), i, i);
                    m_passengers.removeAt(i);
                    endRemoveRows();
                }
            }
            // then, move the current passagers so the list is in the same order as the new list

            for (int i = 0; i < m_passengers.size(); ++i)
            {
                if (m_passengers[i] != newPassengers[i])
                {
                    // move the passenger
                    int row = newPassengers.indexOf(m_passengers[i]);
                    beginMoveRows(QModelIndex(), i, i, QModelIndex(), row);
                    m_passengers.move(i, row);
                    endMoveRows();
                }
            }

            // finally, update the passengers that are in both lists if the updateDateDate has changed

            for (int i = 0; i < m_passengers.size(); ++i)
            {
                if (m_passengers[i].updateDate() != newPassengers[i].updateDate())
                {
                    // update the passenger
                    m_passengers[i] = newPassengers[i];
                    QModelIndex topLeft = index(i, 0);
                    QModelIndex bottomRight = index(i, 0);
                    emit dataChanged(topLeft, bottomRight);
                }
            }

            return;
        });
    });

    connect(EventDispatcher::instance()->passenger(), &PassengerSignals::removed, this, [this](QList<int> dtoList) {
        for (int dtoId : dtoList)
        {
            int position = m_passengerIds.indexOf(dtoId);
            if (position != -1)
            {
                beginRemoveRows(QModelIndex(), position, position);
                m_passengers.removeAt(position);
                m_passengerIds.removeAt(position);
                endRemoveRows();
            }
        }
    });

    connect(EventDispatcher::instance()->passenger(), &PassengerSignals::insertedIntoCarPassengers, this,
            [this](PassengerInsertedIntoRelativeDTO dto) {
                // remove passenger from this model if dto.passenger().id() is here
                // add passenger to this model if dto.relatedId() is here

                int passengerId = dto.passenger().id();
                if (m_passengerIds.contains(passengerId) && m_carId != dto.relatedId())
                {
                    int position = m_passengerIds.indexOf(passengerId);
                    beginRemoveRows(QModelIndex(), position, position);
                    m_passengers.removeAt(position);
                    m_passengerIds.removeAt(position);
                    endRemoveRows();
                }
                else if (!m_passengerIds.contains(passengerId) && m_carId == dto.relatedId())
                {
                    int position = dto.position();
                    beginInsertRows(QModelIndex(), position, position);
                    m_passengers.insert(position, dto.passenger());
                    m_passengerIds.insert(position, passengerId);
                    endInsertRows();
                }
            });

    connect(EventDispatcher::instance()->passenger(), &PassengerSignals::updated, this, [this](PassengerDTO dto) {
        for (int i = 0; i < m_passengers.size(); ++i)
        {
            if (m_passengers.at(i).id() == dto.id())
            {
                m_passengers[i] = dto;
                m_passengerIds[i] = dto.id();
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

    return m_passengers.count();
}

QVariant PassengerListModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

    int row = index.row();
    if (row >= m_passengers.size())
        return QVariant();

    if (role == Qt::DisplayRole)
    {
        return m_passengers.at(row).name();
    }

    if (role == Id)
        return m_passengers.at(row).id();
    if (role == Name)
        return m_passengers.at(row).name();

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
    if (row >= m_passengers.size())
        return false;

    if (role == Qt::EditRole)
    {
        m_passengers[row].setName(value.toString());

        UpdatePassengerDTO dto;
        dto.setId(m_passengers[row].id());
        dto.setName(m_passengers[row].name());

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
    m_passengers.clear();
    m_passengerIds.clear();

    auto task = Car::CarController::instance()->getWithDetails(m_carId);
    QCoro::connect(std::move(task), this, [this](auto &&result) {
        const QList<Simple::Contracts::DTO::Passenger::PassengerDTO> passengers = result.passengers();
        beginInsertRows(QModelIndex(), 0, passengers.size() - 1);
        m_passengers = passengers;
        // fill m_passengerIds
        for (const auto &passenger : passengers)
        {
            m_passengerIds.append(passenger.id());
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
        m_passengers.clear();
        m_passengerIds.clear();
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
