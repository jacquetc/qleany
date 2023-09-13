#pragma once

#include "passenger/passenger_dto.h"
#include "presenter_export.h"
#include <QAbstractListModel>

class SIMPLEEXAMPLE_PRESENTER_EXPORT PassengerListModel : public QAbstractListModel
{
    Q_OBJECT
    Q_PROPERTY(int carId READ carId WRITE setCarId RESET resetCarId NOTIFY carIdChanged FINAL)

  public:
    enum Roles
    {
        // treeItems :
        Id = Qt::UserRole,
        Name = Qt::UserRole + 1
    };
    Q_ENUM(Roles)

    explicit PassengerListModel(QObject *parent = nullptr);

    // Header:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    // Basic functionality:
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    int carId() const;
    void setCarId(int newCarId);
    void resetCarId();

    Qt::ItemFlags flags(const QModelIndex &index) const override;
    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;

  signals:
    void carIdChanged();

  private:
    void populate();

    QList<Simple::Contracts::DTO::Passenger::PassengerDTO> m_passengers;
    QList<int> m_passengerIds;
    int m_carId = 0;

    // QAbstractItemModel interface
  public:
    QHash<int, QByteArray> roleNames() const override;
};
