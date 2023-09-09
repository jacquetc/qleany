#include "passenger_list_model.h"
#include "car/car_controller.h"
#include "event_dispatcher.h"
#include "passenger/passenger_controller.h"
#include <QCoroTask>

using namespace Simple::Controller;

PassengerListModel::PassengerListModel(QObject *parent) : QAbstractListModel(parent)
{

    connect(EventDispatcher::instance()->passenger(), &PassengerSignals::created, this, [this](PassengerDTO dto) {
        beginInsertRows(QModelIndex(), rowCount(), rowCount());
        m_passengers.append(dto);
        endInsertRows();
    });

    connect(EventDispatcher::instance()->passenger(), &PassengerSignals::updated, this, [this](PassengerDTO dto) {
        for (int i = 0; i < m_passengers.size(); ++i)
        {
            if (m_passengers.at(i).id() == dto.id())
            {
                m_passengers[i] = dto;
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
    auto task = Car::CarController::instance()->getWithDetails(m_carId);
    QCoro::connect(std::move(task), this, [this](auto &&result) {
        const QList<Simple::Contracts::DTO::Passenger::PassengerDTO> passengers = result.passengers();
        beginInsertRows(QModelIndex(), 0, passengers.size() - 1);
        m_passengers = passengers;
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
