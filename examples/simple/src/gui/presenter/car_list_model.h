// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car/car_dto.h"
#include "presenter_export.h"
#include <QAbstractListModel>

using namespace Simple::Contracts::DTO::Car;

namespace Simple::Presenter
{
class SIMPLEEXAMPLE_PRESENTER_EXPORT CarListModel : public QAbstractListModel
{
    Q_OBJECT

  public:
    enum Roles
    {

        IdRole = Qt::UserRole + 0,
        UuidRole = Qt::UserRole + 1,
        CreationDateRole = Qt::UserRole + 2,
        UpdateDateRole = Qt::UserRole + 3,
        ContentRole = Qt::UserRole + 4
    };
    Q_ENUM(Roles)

    explicit CarListModel(QObject *parent = nullptr);

    // Header:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    // Basic functionality:
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    Qt::ItemFlags flags(const QModelIndex &index) const override;
    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;
    QHash<int, QByteArray> roleNames() const override;

  signals:

  private:
    void populate();

    QList<CarDTO> m_carList;
    QList<int> m_carIdList;
};

} // namespace Simple::Presenter